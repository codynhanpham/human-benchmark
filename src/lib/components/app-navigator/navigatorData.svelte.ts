import type { Component } from "svelte";

import type { IconProps, IconNode } from "@lucide/svelte";
import {
    Blocks,
    LayoutDashboard,
} from '@lucide/svelte/icons';

import {
    HouseFilled,
    AimTrainer,
    NumberMemory,
    ReactionTime,
    SequenceMemory,
    Typing,
    VerbalMemory
} from '$lib/icons';

export type NavigationRoutesType = {
    name: string;
    url: string;
    icon?: Component<IconProps, {}, ""> | IconNode;
    iconAttributes?: IconProps;
    ogWebsite?: string;
}
// A function to assert that a value is of type NavigationRoutesType
export function asNavigationRoute(value: any): NavigationRoutesType {
    return value as NavigationRoutesType;
}

export type NavSeparatorType = {
    name: "separator";
}

export const NavigationRoutes: (NavigationRoutesType | NavSeparatorType)[] = [
    {
        name: "Home",
        url: "/",
        icon: HouseFilled,
        iconAttributes: { class: "!size-4.5"},
        ogWebsite: "https://humanbenchmark.com",
    },
    { name: "separator" },
    {
        name: "Reaction Time",
        url: "/reaction-time",
        icon: ReactionTime,
        iconAttributes: { viewBox: "0 0 100 128" },
        ogWebsite: "https://humanbenchmark.com/tests/reactiontime",
    },
    {
        name: "Sequence Memory",
        url: "/sequence-memory",
        icon: SequenceMemory,
        iconAttributes: { viewBox: "0 0 128 128" },
        ogWebsite: "https://humanbenchmark.com/tests/sequence",
    },
    {
        name: "Aim Trainer",
        url: "/aim-trainer",
        icon: AimTrainer,
        iconAttributes: { viewBox: "0 0 128 128" },
        ogWebsite: "https://humanbenchmark.com/tests/aim",
    },
    {
        name: "Number Memory",
        url: "/number-memory",
        icon: NumberMemory,
        iconAttributes: { viewBox: "0 0 128 128" },
        ogWebsite: "https://humanbenchmark.com/tests/number-memory",
    },
    {
        name: "Verbal Memory",
        url: "/verbal-memory",
        icon: VerbalMemory,
        iconAttributes: { viewBox: "0 0 100 128" },
        ogWebsite: "https://humanbenchmark.com/tests/verbal-memory",
    },
    {
        name: "Chimp Test",
        url: "/chimp-test",
        icon: Blocks,
        iconAttributes: { class: "!size-5" },
        ogWebsite: "https://humanbenchmark.com/tests/chimp",
    },
    {
        name: "Visual Memory",
        url: "/visual-memory",
        icon: LayoutDashboard,
        iconAttributes: { fill: "currentColor", class: "!size-4.5" },
        ogWebsite: "https://humanbenchmark.com/tests/memory",
    },
    {
        name: "Typing",
        url: "/typing",
        icon: Typing,
        iconAttributes: { viewBox: "0 0 125 128" },
        ogWebsite: "https://humanbenchmark.com/tests/typing",
    },
];