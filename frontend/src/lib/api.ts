// src/lib/api.ts
class ApiClient {
    public baseUrl: string;

    constructor(baseUrl?: string) {
        this.baseUrl = baseUrl || import.meta.env.VITE_API_URL || 'http://localhost:8080';
    }

    setBaseUrl(url: string) {
        this.baseUrl = url;
    }

    async get(path: string) {
        const response = await fetch(`${this.baseUrl}${path}`, {
            credentials: 'include', // Important for cookies/session
        });
        if (!response.ok) throw new Error(`API request to ${this.baseUrl}${path} failed`);
        return response;
    }

    async post(path: string, data: any) {
        const response = await fetch(`${this.baseUrl}${path}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });
        if (!response.ok) throw new Error('API request failed');
        return response;
    }

    // Auth specific methods
    async initiateLogin() {
        window.location.href = `${this.baseUrl}/auth/login`;
    }

    async checkAuth() {
        console.log(`Checking auth, fetching ${this.baseUrl}/api/user`);
        const response = await fetch(`${this.baseUrl}/api/user`, {
            credentials: 'include',
        });
        if (!response.ok) throw new Error('Not authenticated');
        return response.json();
    }

    async logout() {
        await fetch(`${this.baseUrl}/api/logout`, {
            method: 'POST',
            credentials: 'include',
        });
    }
}

// Create a singleton instance
export const api = new ApiClient();