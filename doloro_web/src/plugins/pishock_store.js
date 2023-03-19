import { defineStore } from "pinia";
import {get_shocker_list, create_shocker, execute_shock} from "@/plugins/pishock";

export const usePishockStore = defineStore("pishock", {
    state: () => ({
        shockers: [],
    }),
    getters: {
        shocker_list: (state) => state.shockers || [],
        shocker_by_id: (state) => (id) => {
            return state.shockers.find(shocker => shocker.id === id)
        },
    },
    actions: {
        async update_shocker_list() {
            let shockers = await get_shocker_list()
            console.log(shockers)
            this.shockers = shockers
            return shockers
        },
        async create_shocker(share_code) {
            await create_shocker(share_code)
            await this.update_shocker_list()
        },
        async execute_shock(shocker_id, intensity, duration, warn) {
            await execute_shock(shocker_id, intensity, duration, warn)
        }
    }
});

// Fetch the shocker list on page load
usePishockStore.call("update_shocker_list")
