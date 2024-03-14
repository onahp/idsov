import { writable } from "svelte/store";

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const notifications = writable([]);
export const weClientStored = writable(null);
export const dAppDna = writable("Getting DNA ...");
export const dProfilesStore = writable(null);
export const dProfilesClient = writable(null);
export const dInitialized = writable(false);
export const dConnected = writable(false);

export function navigate(location, hash) {
    view.update(v => location);
    viewHash.update(h => hash)
}

export function notifications_update(new_notifications) {
    let ordered = new_notifications.sort((a, b) => parseFloat(b.timestamp) - parseFloat(a.timestamp));
    notifications.update(v => ordered)
}

export function setWeClient(client) {
    weClientStored.update(v => client);
}

export function setDna(dna) {
  dAppDna.update(v => dna);
}

export function setProfilesStore(profilesStore) {
  dProfilesStore.update(v => profilesStore);
}

export function setProfilesClient(profilesClient) {
  dProfilesClient.update(v => profilesClient);
}

export function setInitialized(initialized) {
  dInitialized.update(v => initialized);
}

export function setConnected(connected) {
  dConnected.update(v => connected);
}
