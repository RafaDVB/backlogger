import { defineStore } from "pinia";
import { ref } from "vue";
import * as system from "../api/system";

export const useDebugStore = defineStore("debug", () => {
    const reply = ref("");

    async function ping(message: string) {
        reply.value = await system.ping(message);
    }

    return { reply, ping };
});
