<template>
    <!-- Small card used to add a new pishocker -->
    <v-card max-width="500">
        <v-card-title>
            Add New PiShocker
        </v-card-title>
        <v-card-text>
            <v-row>
                <v-text-field
                    v-model="sharecode"
                    label="PiShock Sharecode"
                    required
                ></v-text-field>
            </v-row>
        </v-card-text>
        <v-card-actions>
            <v-btn color="green" variant="elevated" @click="pishockStore.create_shocker(sharecode)">Add<v-icon end icon="mdi-plus"></v-icon></v-btn>
        </v-card-actions>
    </v-card>
</template>

<script>
import {usePishockStore} from "@/plugins/pishock_store";

export default {
    name: "NewPishocker",
    data() {
        return {
            dialog: false,
            sharecode: "",

        }
    },
    setup() {
        const pishockStore = usePishockStore();
        return {
            pishockStore
        };
    },
}
</script>

<style scoped>

</style>
