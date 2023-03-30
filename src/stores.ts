import { writable } from 'svelte/store'

export enum Tab {
  Home,
  Help,
}

export const currentTab = writable(Tab.Home)
