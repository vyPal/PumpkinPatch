<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button, Card } from 'flowbite-svelte';
	let { children } = $props();
	import { auth } from '$lib/stores/auth';
    import { api } from '$lib/api';
	import { onMount } from 'svelte';
	import { ArrowUpRightFromSquareOutline } from 'flowbite-svelte-icons';
	import { goto } from '$app/navigation';

    onMount(() => {
        auth.init();
    });

    async function logout() {
        await api.logout();
        auth.clear();
        goto('/');
    }

	function login() {
        api.initiateLogin();
    }
</script>
  
<Navbar rounded color="form" class="fixed top-0 right-0 left-0 z-50 w-auto" style="margin: 25px 5rem 0 5rem;">
	<NavBrand href="/">
		<img src="/pumpkin.png" class="me-3 h-6 sm:h-9" alt="Pumpkin Logo" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white" style="margin-right: -125px;">Pumpkin Patch</span>
	</NavBrand>
	<NavHamburger />
	<NavUl>
		<NavLi href="/">Home</NavLi>
		<NavLi href="/discover">Discover</NavLi>
		<NavLi href="/docs/components/navbar">About</NavLi>
	</NavUl>
	<div>
		{#if $auth !== null}
			<a href="/dashboard" class="inline-flex items-center text-primary-600 hover:underline mr-3">
				Dashboard
			</a>
			<Button size="sm" on:click={logout}>Log out</Button>
		{:else}
			<Button size="sm" on:click={login}>Log in</Button>
		{/if}
	</div>
</Navbar>

<span class="block" style="height: 5.5rem;"></span>

{@render children()}
