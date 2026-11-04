#!/usr/bin/env node
// Generates THIRD_PARTY_LICENSES.md from the resolved Rust and JavaScript
// dependency trees. Run with `pnpm licenses:generate`; use
// `pnpm licenses:check` in CI to fail when the checked-in notices are out of date.
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outputPath = path.join(root, "THIRD_PARTY_LICENSES.md");
const checkOnly = process.argv.includes("--check");

const LICENSE_FILE = /^(licen[cs]e|copying|notice|unlicense)([._-].*)?$/i;

function readText(file) {
    return fs
        .readFileSync(file, "utf8")
        .replace(/^\uFEFF/, "")
        .replace(/\r\n?/g, "\n")
        .trim();
}

function licenseFilesIn(dir) {
    let entries;
    try {
        entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
        return [];
    }
    return entries
        .filter((entry) => entry.isFile() && LICENSE_FILE.test(entry.name))
        .map((entry) => path.join(dir, entry.name))
        .sort();
}

function addText(pkg, text) {
    const normalized = text.trim();
    if (!normalized) return;
    const hash = createHash("sha256").update(normalized).digest("hex").slice(0, 12);
    if (!texts.has(hash)) {
        texts.set(hash, { text: normalized, usedBy: new Set() });
    }
    texts.get(hash).usedBy.add(`${pkg.name}@${pkg.version}`);
    if (!pkg.textHashes.includes(hash)) pkg.textHashes.push(hash);
}

function addTexts(pkg, files) {
    for (const file of files) {
        try {
            addText(pkg, readText(file));
        } catch {
            // Ignore unreadable license files.
        }
    }
}

function normalizeRepository(value) {
    if (!value) return "";
    const raw = typeof value === "string" ? value : value.url;
    if (!raw) return "";
    return raw
        .replace(/^git\+/, "")
        .replace(/^git@([^:]+):/, "https://$1/")
        .replace(/\.git(#.*)?$/, "")
        .replace(/#.*$/, "");
}

function licenseExpression(value) {
    if (!value) return "UNKNOWN";
    if (typeof value === "string") return value;
    if (Array.isArray(value)) {
        return value.map((entry) => entry.type ?? String(entry)).join(" OR ");
    }
    return String(value);
}

function comparePackages(a, b) {
    return a.name.localeCompare(b.name) || a.version.localeCompare(b.version, undefined, { numeric: true });
}

// ---- Rust packages -------------------------------------------------------

const metadata = JSON.parse(
    execFileSync(
        "cargo",
        ["metadata", "--format-version", "1", "--manifest-path", path.join(root, "src-tauri/Cargo.toml")],
        { encoding: "utf8", maxBuffer: 256 * 1024 * 1024 },
    ),
);

const rustPackages = metadata.packages
    .filter((pkg) => pkg.name !== "backlogger")
    .map((pkg) => {
        const dir = path.dirname(pkg.manifest_path);
        const files = licenseFilesIn(dir);
        if (pkg.license_file) {
            const explicit = path.resolve(dir, pkg.license_file);
            if (fs.existsSync(explicit) && !files.includes(explicit)) files.push(explicit);
        }
        return {
            name: pkg.name,
            version: pkg.version,
            license: licenseExpression(pkg.license) || (pkg.license_file ? `SEE LICENSE IN ${pkg.license_file}` : "UNKNOWN"),
            repository: normalizeRepository(pkg.repository),
            authors: Array.isArray(pkg.authors) ? pkg.authors.filter(Boolean).join(", ") : "",
            files,
            textHashes: [],
        };
    })
    .sort(comparePackages);

// ---- JavaScript packages -------------------------------------------------

function walkPackageJson(dir, depth = 0) {
    if (depth > 3) return [];
    let entries;
    try {
        entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
        return [];
    }
    const found = [];
    for (const entry of entries) {
        const full = path.join(dir, entry.name);
        if (entry.isSymbolicLink()) continue;
        if (!entry.isDirectory()) continue;
        if (entry.name === "node_modules") {
            found.push(...walkPackageJson(full, depth + 1));
            continue;
        }
        const manifest = path.join(full, "package.json");
        if (fs.existsSync(manifest)) {
            found.push({ dir: full, manifest });
        } else {
            found.push(...walkPackageJson(full, depth + 1));
        }
    }
    return found;
}

const jsPackages = new Map();
const pnpmDir = path.join(root, "node_modules/.pnpm");
if (fs.existsSync(pnpmDir)) {
    for (const entry of fs.readdirSync(pnpmDir, { withFileTypes: true })) {
        if (!entry.isDirectory()) continue;
        for (const candidate of walkPackageJson(path.join(pnpmDir, entry.name, "node_modules"))) {
            let pkg;
            try {
                pkg = JSON.parse(fs.readFileSync(candidate.manifest, "utf8"));
            } catch {
                continue;
            }
            if (!pkg.name || !pkg.version) continue;
            if (pkg.name === "backlogger") continue;
            const key = `${pkg.name}@${pkg.version}`;
            if (jsPackages.has(key)) continue;
            const author =
                typeof pkg.author === "string"
                    ? pkg.author
                    : pkg.author?.name
                      ? pkg.author.name
                      : Array.isArray(pkg.contributors)
                        ? pkg.contributors
                              .map((person) => (typeof person === "string" ? person : person?.name))
                              .filter(Boolean)
                              .join(", ")
                        : "";
            jsPackages.set(key, {
                name: pkg.name,
                version: pkg.version,
                license: licenseExpression(pkg.license ?? pkg.licenses),
                repository: normalizeRepository(pkg.repository ?? pkg.homepage),
                authors: author,
                files: licenseFilesIn(candidate.dir),
                textHashes: [],
            });
        }
    }
}

const texts = new Map();
const jsList = [...jsPackages.values()].sort(comparePackages);
const packages = [...rustPackages, ...jsList];
for (const pkg of packages) {
    addTexts(pkg, pkg.files);
}

// Some packages declare a license in their manifest but do not ship the text
// in the published archive. Reuse the canonical text of a matching dependency
// where possible so the notice file is still self-contained.

function findLicenseBody(phrase) {
    for (const record of texts.values()) {
        const index = record.text.indexOf(phrase);
        if (index !== -1) return record.text.slice(index).trim();
    }
    return "";
}

const canonicalBodies = {
    MIT: findLicenseBody("Permission is hereby granted, free of charge"),
    "BSD-3-Clause": findLicenseBody("Redistribution and use in source and binary forms"),
    "Apache-2.0": findLicenseBody("Apache License"),
    Zlib: findLicenseBody("This software is provided 'as-is'") || findLicenseBody('This software is provided "as-is"'),
    "MPL-2.0": findLicenseBody("Mozilla Public License Version 2.0"),
};
const preferredLicenses = ["MIT", "Apache-2.0", "BSD-3-Clause", "Zlib", "MPL-2.0"];

function pickLicense(expression) {
    const parts = expression.split(/\s+OR\s+|\s*\/\s*|\s+AND\s+/);
    return preferredLicenses.find((id) => parts.includes(id)) ?? "";
}

for (const pkg of packages) {
    if (pkg.textHashes.length > 0) continue;
    const id = pickLicense(pkg.license);
    const body = id ? canonicalBodies[id] : "";
    if (!body) continue;

    const authors = pkg.authors || `${pkg.name} contributors`;
    const heading =
        id === "MIT"
            ? "MIT License"
            : id === "BSD-3-Clause"
              ? "BSD 3-Clause License"
              : id === "Zlib"
                ? "zlib License"
                : `${id} License`;
    const copyright = id === "MIT" || id === "BSD-3-Clause" || id === "Zlib" ? `Copyright (c) ${authors}\n\n` : "";
    addText(pkg, `${heading}\n\n${copyright}${body}`);
}

// ---- Render --------------------------------------------------------------

const missing = packages.filter((pkg) => pkg.textHashes.length === 0);
const licenseCounts = new Map();
for (const pkg of packages) {
    const key = pkg.license || "UNKNOWN";
    licenseCounts.set(key, (licenseCounts.get(key) ?? 0) + 1);
}

const lines = [
    "# Third-party licenses",
    "",
    "Backlogger is licensed under the GNU General Public License, version 3 or",
    "later. See [LICENSE](LICENSE) for the project license.",
    "",
    "This file lists the third-party software used by Backlogger and",
    "reproduces the license texts and notices that require attribution.",
    "It is generated from `Cargo.lock` and the installed pnpm packages by",
    "`pnpm licenses:generate`.",
    "",
    `It covers ${rustPackages.length} Rust crates and ${jsPackages.size} JavaScript packages.`,
    "",
    "## License summary",
    "",
    "| License | Packages |",
    "| --- | ---: |",
    ...[...licenseCounts.entries()]
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([license, count]) => `| \`${license.replaceAll("|", "\\|")}\` | ${count} |`),
    "",
    "## Bundled assets",
    "",
    "- Pixel Operator and Pixel Operator Bold by Jayvee Enaguas (HarvettFox96),",
    "  licensed under Creative Commons Zero (CC0) 1.0. The font files retain",
    "  their copyright and license metadata.",
    "",
    "## Packages",
    "",
];

function renderGroup(title, group) {
    lines.push(`### ${title}`, "");
    for (const pkg of [...group].sort(comparePackages)) {
        lines.push(`#### \`${pkg.name}\` ${pkg.version}`, "");
        lines.push(`- License: \`${pkg.license}\``);
        if (pkg.repository) lines.push(`- Source: <${pkg.repository}>`);
        if (pkg.textHashes.length === 0) {
            lines.push("- License text: not included in the package; see the SPDX identifier above.");
        } else {
            const links = pkg.textHashes.map((hash) => `[text ${hash}](#license-text-${hash})`);
            lines.push(`- License text: ${links.join(", ")}`);
        }
        lines.push("");
    }
}

renderGroup("Rust crates", rustPackages);
renderGroup("JavaScript packages", jsList);

if (missing.length > 0) {
    lines.push("## Packages without a bundled license file", "");
    lines.push("These packages declare a license but do not ship a license file in their distribution. Their SPDX identifiers are listed above.", "");
    for (const pkg of [...missing].sort(comparePackages)) {
        lines.push(`- \`${pkg.name}\` ${pkg.version} — \`${pkg.license}\``);
    }
    lines.push("");
}

lines.push("## License texts", "");
for (const [hash, record] of [...texts.entries()].sort(([a], [b]) => a.localeCompare(b))) {
    lines.push(`<a id="license-text-${hash}"></a>`, "", `### License text ${hash}`, "");
    lines.push("````text", record.text, "````", "");
}

const output = `${lines.join("\n").replace(/\n{3,}/g, "\n\n")}\n`;

if (checkOnly) {
    const current = fs.existsSync(outputPath) ? fs.readFileSync(outputPath, "utf8") : "";
    if (current !== output) {
        console.error("THIRD_PARTY_LICENSES.md is out of date. Run `pnpm licenses:generate` and commit the result.");
        process.exit(1);
    }
    console.log("Third-party license notices are up to date.");
} else {
    fs.writeFileSync(outputPath, output);
    console.log(
        `Wrote ${path.relative(root, outputPath)} (${packages.length} packages, ${texts.size} unique license texts, ${missing.length} packages without a license file).`,
    );
}
