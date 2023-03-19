<template>
    <v-container fluid>
        <v-row dense>
            <v-col cols="12" sm="4" md="4">
                <PiShockerCard v-for="shocker in pishockStore.shocker_list" :shockerid="shocker.id" :key="shocker.id" />
            </v-col>
        </v-row>
    </v-container>
</template>

<script>
import PiShockerCard from "@/components/PiShockerCard.vue";
import {usePishockStore} from "@/plugins/pishock_store";

export default {
    name: "PiShockerList",
    components: {PiShockerCard},
    setup() {
        // load pishock store
        const pishockStore = usePishockStore();
        return {
            pishockStore
        };
    },
    async mounted() {
        await this.pishockStore.update_shocker_list();
    },
}
</script>

<style scoped>

</style>
