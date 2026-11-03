import type { BackgroundId } from "./appearance";

interface Star {
    x: number;
    y: number;
    size: number;
    start: number;
    duration: number;
}

interface Drop {
    x: number;
    y: number;
    size: number;
    speed: number;
    phase: number;
}

export interface Background {
    setMode(mode: BackgroundId): void;
    resize(width: number, height: number): void;
    pause(): void;
    resume(): void;
    destroy(): void;
}

const RAIN_SPEED = 1.2;
const DEPTHS_SPEED = 0.8;
const MAX_DROP_ALPHA = 0.3;
const OCEAN_CELL = 16;
const OCEAN_SQUARE = 12;
const OCEAN_FRAME_MS = 33;

const random = (min: number, max: number) => min + Math.random() * (max - min);

export function createBackground(canvas: HTMLCanvasElement, options: { animate: boolean }): Background {
    const context = canvas.getContext("2d");
    if (!context) {
        return { setMode() { }, resize() { }, pause() { }, resume() { }, destroy() { } };
    }
    const ctx: CanvasRenderingContext2D = context;

    let width = 0;
    let height = 0;
    let mode: BackgroundId = "off";
    let stars: Star[] = [];
    let drops: Drop[] = [];
    let raf = 0;
    let last = 0;
    let lastDrawn = 0;
    let paused = false;

    function spawnStar(star: Star, now: number, firstTime: boolean) {
        star.duration = random(2500, 7000);
        star.start = firstTime ? now - random(0, star.duration) : now;
        star.x = Math.floor(random(0, width));
        star.y = Math.floor(random(0, height));
        star.size = Math.round(random(2, 6));
    }

    function createDrop(): Drop {
        const size = 3 + Math.round(Math.pow(Math.random(), 2) * 21);
        return {
            x: Math.floor(random(0, width)),
            y: random(0, height),
            size,
            speed: 20 + size * 6,
            phase: random(0, Math.PI * 2),
        };
    }

    function populate() {
        const now = performance.now();
        stars = Array.from({ length: Math.round((width * height) / 9000) }, () => {
            const star: Star = { x: 0, y: 0, size: 0, start: 0, duration: 0 };
            spawnStar(star, now, true);
            return star;
        });
        drops = Array.from({ length: Math.round(width / 18) }, createDrop);
    }

    function drawStars(now: number) {
        for (const star of stars) {
            if (now >= star.start + star.duration) spawnStar(star, now, false);
            ctx.globalAlpha = Math.max(0, Math.sin((Math.PI * (now - star.start)) / star.duration)) * 0.6;
            ctx.fillRect(star.x, star.y, star.size, star.size);
        }
    }

    function drawDrops(now: number, dt: number, rising: boolean) {
        const factor = rising ? DEPTHS_SPEED : RAIN_SPEED;
        for (const drop of drops) {
            drop.y += (rising ? -1 : 1) * drop.speed * factor * dt;
            if (rising ? drop.y < -drop.size : drop.y > height) {
                drop.y = rising ? height : -drop.size;
                drop.x = Math.floor(random(0, width));
            }
            const sway = rising ? Math.round(Math.sin((now / 1000) * 0.8 + drop.phase) * 4) : 0;
            ctx.globalAlpha = Math.min(MAX_DROP_ALPHA, 0.04 + drop.size * 0.012);
            ctx.fillRect(drop.x + sway, Math.floor(drop.y), drop.size, drop.size);
        }
    }

    function drawOcean(now: number) {
        const t = now / 1000;
        for (let y = 0; y < height; y += OCEAN_CELL) {
            for (let x = 0; x < width; x += OCEAN_CELL) {
                const wave =
                    (Math.sin(x * 0.012 + t * 1.1) + Math.sin(y * 0.02 - t * 0.8) + Math.sin((x + y) * 0.008 + t * 0.5) + 3) / 6;
                ctx.globalAlpha = 0.03 + (Math.round(wave * 6) / 6) * 0.17;
                ctx.fillRect(x, y, OCEAN_SQUARE, OCEAN_SQUARE);
            }
        }
    }

    function schedule() {
        if (options.animate && !paused && mode !== "off") raf = requestAnimationFrame(frame);
    }

    function frame(now: number) {
        if (mode === "ocean" && now - lastDrawn < OCEAN_FRAME_MS) {
            schedule();
            return;
        }
        const dt = Math.min(0.05, (now - last) / 1000);
        last = now;
        lastDrawn = now;

        ctx.clearRect(0, 0, width, height);
        ctx.fillStyle = "#ffffff";
        if (mode === "stars") drawStars(now);
        else if (mode === "rain") drawDrops(now, dt, false);
        else if (mode === "depths") drawDrops(now, dt, true);
        else if (mode === "ocean") drawOcean(now);
        ctx.globalAlpha = 1;

        schedule();
    }

    function restart() {
        cancelAnimationFrame(raf);
        if (mode === "off") {
            ctx.clearRect(0, 0, width, height);
            return;
        }
        last = performance.now();
        lastDrawn = 0;
        raf = requestAnimationFrame(frame);
    }

    return {
        setMode(next) {
            mode = next;
            restart();
        },
        resize(newWidth, newHeight) {
            width = newWidth;
            height = newHeight;
            canvas.width = width;
            canvas.height = height;
            populate();
            restart();
        },
        pause() {
            paused = true;
            cancelAnimationFrame(raf);
        },
        resume() {
            paused = false;
            restart();
        },
        destroy() {
            paused = true;
            cancelAnimationFrame(raf);
        },
    };
}