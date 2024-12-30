// src/routes/auth/callback/+page.ts
import { api } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url }) => {
    const code = url.searchParams.get('code');
    
    if (!code) {
        return redirect(303, '/');
    }

    return redirect(303, `${api.baseUrl}/auth/callback?code=${code}`);
};