import { writable } from 'svelte/store';

type UserInfo = {
  email?: string;
  name?: string;
};

export const userInfo = writable<UserInfo>({email: undefined, name: undefined});

export async function getSession() {
  console.log('Fetching session');
  const res = await fetch('/auth/session', {credentials: 'same-origin'});
  let sessionResponse = await res.json();
  if (sessionResponse.email !== '') {
    userInfo.set({email: sessionResponse.email, name: sessionResponse.name});
  } else {
    userInfo.set({email: undefined, name: undefined});
  }
}
