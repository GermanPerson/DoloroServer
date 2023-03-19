<script>
import {execute_shock, execute_vibrate} from "@/plugins/pishock";
import {usePishockStore} from "@/plugins/pishock_store";

export default {
    name: "PiShockerCard",
    methods: {execute_vibrate, execute_shock},
    props: ["shockerid"],
    computed: {
     shocker() {
         return this.pishockStore.shocker_by_id(this.shockerid);
     }
    },
    setup() {
        const pishockStore = usePishockStore();
        return {
            pishockStore
        };
    }
}
</script>

<template>
    <v-card max-width="500">
        <v-card-title>
            <v-icon icon="mdi-lightning-bolt" />
            {{shocker.name}}
        </v-card-title>
        <v-card-text>
            <v-row>
                <v-col cols="12">
                    <p>Max Intensity: {{shocker.max_intensity}}%</p>
                    <p>Max Duration: {{shocker.max_duration}}s</p>
                </v-col>
            </v-row>
        </v-card-text>
        <v-card-actions>
            <v-btn color="green" variant="elevated" @click="execute_shock(shockerid, 15, 1, true)">Shock<v-icon end icon="mdi-flash-alert"></v-icon></v-btn>
            <v-btn color="green" variant="elevated" @click="execute_vibrate(shockerid, 50, 1)">Vibrate<v-icon end icon="mdi-vibrate"></v-icon></v-btn>

        </v-card-actions>
    </v-card>

</template>

<style scoped>

</style>
