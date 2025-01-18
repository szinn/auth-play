<script lang="ts">
    import { getLogout } from '$lib/auth/auth';

    async function handleLogout() {
        await getLogout();
        return {
            status: 302,
            redirect: '/app'
        };
    }
</script>

<div>
    <button on:click={handleLogout}> Logout </button>
</div>
