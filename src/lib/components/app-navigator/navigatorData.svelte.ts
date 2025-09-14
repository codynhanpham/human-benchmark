import type { Component } from "svelte";

import type { IconProps } from "@lucide/svelte";
import {
    ChevronLeft,
    ChevronRight,
    CirclePlus,
    History,
    Home,
    LibraryBig,
    RotateCw,
    Settings,
} from '@lucide/svelte/icons';

export type NavigationRoutesType = {
    name: string;
    url: string;
    icon?: Component<IconProps, {}, "">;
}

export const NavigationRoutes: NavigationRoutesType[] = [
    {
        name: "Home",
        url: "/",
        icon: Home
    },
    {
        name: "Library",
        url: "/library",
        icon: LibraryBig
    },
    {
        name: "Create",
        url: "/create",
        icon: CirclePlus
    },
    {
        name: "History",
        url: "/history",
        icon: History
    },
    {
        name: "Settings",
        url: "/settings",
        icon: Settings
    }
];