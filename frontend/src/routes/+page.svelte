<script>
  import { Button, Card } from 'flowbite-svelte';
	import { ArrowUpRightFromSquareOutline } from 'flowbite-svelte-icons';
  import { api } from '$lib/api';

	function login() {
    api.initiateLogin();
  }
</script>

<div class="flex flex-col absolute right-0 left-0" style="padding: 0 25px 0 25px;">
    <Card class="text-center place-self-center" size="lg" padding="xl">
        <h5 class="mb-2 text-3xl font-bold text-gray-900 dark:text-white">Pumpkin Patch</h5>
        <p class="mb-5 text-base text-gray-500 sm:text-lg dark:text-gray-400">The official plugin marketplace for Pumpkin</p>
        <div class="justify-center items-center space-y-4 sm:flex sm:space-y-0 sm:space-x-6 rtl:space-x-reverse">
            <Button onclick={() => window.location.href = '/discover'} color="primary">Discover plugins</Button>
            <a href="#idk" on:click={login} class="inline-flex items-center text-primary-600 hover:underline">
                Sign up
                <ArrowUpRightFromSquareOutline class="w-4 h-4 ms-2.5" />
            </a>
        </div>
    </Card>
</div>