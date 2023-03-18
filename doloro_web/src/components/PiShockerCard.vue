<template>
    <v-card max-width="500" v-if="!loading">
        <v-card-title>
            <v-icon icon="mdi-lightning-bolt" />
            {{shockerInfo.name}}
        </v-card-title>
        <v-card-text>
            <v-row>
                <v-col cols="12">
                    <p>Max Intensity: {{shockerInfo.max_intensity}}%</p>
                    <p>Max Duration: {{shockerInfo.max_duration}}s</p>
                </v-col>
            </v-row>
        </v-card-text>
        <v-card-actions>
            <v-btn color="green" variant="elevated" @click="execute_shock(shockerInfo.id, 15, 1, true)">Shock<v-icon end icon="mdi-flash-alert"></v-icon></v-btn>
            <v-btn color="green" variant="elevated" @click="execute_vibrate(shockerInfo.id, 50, 1)">Shock<v-icon end icon="mdi-vibrate"></v-icon></v-btn>

        </v-card-actions>
    </v-card>

</template>

<script>
import {execute_shock, execute_vibrate, get_shocker_info} from "@/plugins/pishock";

export default {
    name: "PiShockerCard",
    methods: {execute_vibrate, execute_shock},
    props: ["shockerid"],
    data: function () {
        return {
            shockerInfo: {},
            loading: true,
        };
    },
    mounted() {
        get_shocker_info(this.shockerid).then((response) => {
            this.shockerInfo = response;
            this.loading = false;
        });
    }
}
</script>

<style scoped>

</style>
