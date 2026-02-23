<script setup lang="ts">
import AvatarCard from "./components/AvatarCard.vue";
</script>

<template>
    <v-app>
        <v-navigation-drawer
            color="grey-lighten-3"
            model-value
            class="pt-3"
            width="auto"
            rail
            permanent
        >
            <v-list class="d-flex flex-column align-center ga-6">
                <avatar-card></avatar-card>
                <v-btn
                    to="/downloads"
                    class="rounded-circle"
                    size="35"
                    variant="plain"
                >
                    <v-icon size="18" class="ma-auto">mdi-download</v-icon>
                </v-btn>
            </v-list>
            <template v-slot:append>
                <v-list class="d-flex flex-column align-center">
                    <v-btn
                        to="/settings"
                        class="rounded-circle"
                        size="35"
                        variant="plain"
                    >
                        <v-icon size="18" class="ma-auto">mdi-cog</v-icon>
                    </v-btn>
                </v-list>
            </template>
        </v-navigation-drawer>

        <v-main>
            <router-view />
        </v-main>
    </v-app>
</template>

<style scoped></style>
