<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from "vue";

const props = defineProps<{ image?: string }>();

const open = ref(false);
const anchor = ref<any>(null);
const popover = ref<HTMLElement | null>(null);
const coords = ref({ top: 0, left: 0 });
const CARD_WIDTH = 200;

function getDom(el: any): HTMLElement | null {
    if (!el) return null;
    // Vuetify components return a component instance; use .$el when present
    return (el.$el as HTMLElement) || (el as HTMLElement) || null;
}

function toggle() {
    open.value = !open.value;
    if (open.value) position();
}

function position() {
    nextTick(() => {
        const el = getDom(anchor.value);
        if (!el) return;
        const r = el.getBoundingClientRect();
        // place the card so its top-left corner is near the avatar's bottom-left
        const padding = 6;
        let left = r.left + window.scrollX + 0; // align left edges
        let top = r.bottom + window.scrollY + padding;

        // keep within viewport horizontally
        const minLeft = 4;
        const maxLeft = Math.max(4, window.innerWidth - CARD_WIDTH - 4);
        if (left < minLeft) left = minLeft;
        if (left > maxLeft) left = maxLeft;

        coords.value.top = top;
        coords.value.left = left;
    });
}

function onDocClick(e: MouseEvent) {
    const target = e.target as Node;
    const anchorEl = getDom(anchor.value);
    const popEl = popover.value;
    const clickedAnchor =
        anchorEl && (anchorEl === target || anchorEl.contains(target));
    const clickedPopover =
        popEl && (popEl === target || popEl.contains(target));
    if (!clickedAnchor && !clickedPopover) open.value = false;
}

function onWindowChange() {
    if (open.value) position();
}

onMounted(() => {
    document.addEventListener("click", onDocClick);
    window.addEventListener("resize", onWindowChange);
    window.addEventListener("scroll", onWindowChange, true);
});
onBeforeUnmount(() => {
    document.removeEventListener("click", onDocClick);
    window.removeEventListener("resize", onWindowChange);
    window.removeEventListener("scroll", onWindowChange, true);
});
</script>

<template>
    <div class="avatar-card d-flex flex-column align-center">
        <v-avatar
            ref="anchor"
            class="pa-0 rounded-circle"
            @click="toggle"
            :image="props.image"
            size="35"
            color="grey-darken-1"
        >
        </v-avatar>

        <teleport to="body">
            <transition name="fade">
                <div
                    v-if="open"
                    ref="popover"
                    :style="{
                        position: 'absolute',
                        top: coords.top + 'px',
                        left: coords.left + 'px',
                        width: CARD_WIDTH + 'px',
                        zIndex: 9999,
                    }"
                >
                    <v-card>
                        <v-card-title class="py-2">用户名称</v-card-title>
                        <v-card-text class="py-2"
                            >这是用户简介或操作项。</v-card-text
                        >
                        <v-card-actions>
                            <v-btn variant="text" size="small">个人资料</v-btn>
                            <v-btn variant="text" size="small">登出</v-btn>
                        </v-card-actions>
                    </v-card>
                </div>
            </transition>
        </teleport>
    </div>
</template>

<style scoped>
.avatar-card {
    width: 100%;
}
.avatar-card v-avatar {
    cursor: pointer;
}
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>
