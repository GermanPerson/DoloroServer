<template>
    <v-container>
        <v-row>
            <v-col cols="12" sm="6" md="4" v-for="shocker in shockerList" :key="shocker.id">
                <PiShockerCard :shockerid="shocker.id" />
            </v-col>
        </v-row>
    </v-container>
</template>

<script>
import {get_shocker_list} from "@/plugins/pishock";
import PiShockerCard from "@/components/PiShockerCard.vue";

export default {
    name: "PiShockerList",
    components: {PiShockerCard},
    data: function() {
        return {
            shockerList: [],
            loading: true,
        };
    },
    mounted() {
        get_shocker_list().then((response) => {
            this.shockerList = response;
            this.loading = false;
        });
    }
}
</script>

<style scoped>

</style>
