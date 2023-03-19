<template>
    <v-container class="pa-4">
    <PiShockerList />
    <v-divider></v-divider>
    <NewPishocker class="pt-4" />

    </v-container>
</template>

<script>
import PiShockerList from "@/components/PiShockerList.vue";
import NewPishocker from "@/components/NewPishocker.vue";

export default {
  name: "MainInterface",
    components: {NewPishocker, PiShockerList}
}
</script>

<style scoped>

</style>
