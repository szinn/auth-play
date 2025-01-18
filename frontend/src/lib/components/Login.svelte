<script lang="ts">
    import { postLogin, userInfo } from '$lib/auth/auth';

    let email: string = '';
    let password: string = '';
    let errorMessage = '';

    async function handleLogin() {
        let loginResponse = await postLogin(email, password);
        if (loginResponse.result === 'error') {
            errorMessage = loginResponse.message;
        } else {
            return {
                status: 302,
                redirect: '/app'
            };
        }
    }
</script>

{#if userInfo.email === undefined}
    {#if errorMessage}
        <div>
            {errorMessage}
        </div>
    {/if}
    <div>
        <container>
            <div>
                <label for="email">Email</label>
                <input class="input" type="email" placeholder="email" bind:value={email} />
                <label for="password">Password</label>
                <input class="input" type="password" placeholder="password" bind:value={password} />
                <button on:click={handleLogin}> Login </button>
            </div>
        </container>
    </div>
{:else}
    <div>
        <container>
            Logged in as: {$userInfo.email} <br />
            Now you may access the <strong>secure area </strong>from the Nav above
        </container>
    </div>
{/if}

<style>
    div {
        margin: 25px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    label {
        width: 210px;
        text-align: left;
    }
</style>
