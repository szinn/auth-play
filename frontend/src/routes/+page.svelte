<script lang="ts">
    import { getSession, userInfo } from '$lib/auth/auth';
    import Login from '$lib/components/Login.svelte';
    import { onMount } from 'svelte';

    let loggedIn = $derived($userInfo.email !== undefined);

    onMount(getSession);
</script>

<h1>Welcome to SvelteKit</h1>
{#if !loggedIn}
    <Login />
{:else}
    <p>Logged in as {$userInfo.name}({$userInfo.email})</p>
{/if}

<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
