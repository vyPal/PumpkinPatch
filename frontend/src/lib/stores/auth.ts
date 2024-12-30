// src/lib/stores/auth.ts
import { api } from '$lib/api';
import { writable } from 'svelte/store';

interface User {
    user_id: number;
    username: string;
    email: string | null;
}

function createAuthStore() {
    const { subscribe, set, update } = writable<User | null>(null);

    return {
        subscribe,
        set,
        clear: () => set(null),
        async init() {
            try {
                const response = await api.get('/api/user');
                if (response.ok) {
                    const user = await response.json();
                    set(user);
                }
            } catch {
                set(null);
            }
        }
    };
}

export const auth = createAuthStore();