// src/routes/dashboard/+page.ts
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { auth } from '$lib/stores/auth';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    if (browser) {
        const unsubscribe = auth.subscribe(user => {
            if (user === null) {
                goto('/');
            }
        });
        unsubscribe();
    }
};