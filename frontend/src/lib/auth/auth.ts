import { writable } from 'svelte/store';

type UserInfo = {
    email?: string;
    name?: string;
};

export const userInfo = writable<UserInfo>({ email: undefined, name: undefined });

export async function getSession() {
    console.log('Fetching session');
    const res = await fetch('/auth/session', { credentials: 'same-origin' });
    let sessionResponse = await res.json();
    console.log('Response: ', sessionResponse);
    if (sessionResponse.email !== null) {
        userInfo.set({ email: sessionResponse.email, name: sessionResponse.name });
    } else {
        userInfo.set({ email: undefined, name: undefined });
    }
}

export async function postLogin(email: string, password: string) {
    const res = await fetch('/auth/login', {
        method: 'POST',
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email: email, password: password })
    });
    return await res.json();
}
