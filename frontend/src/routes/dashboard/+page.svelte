<!-- src/routes/dashboard/+page.svelte -->
<script lang="ts">
    import { auth } from '$lib/stores/auth';
	import { Avatar, Card, Checkbox, ImagePlaceholder, Input, Popover, Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import { BellOutline, ChartLineUpOutline, ExclamationCircleOutline, FolderOpenOutline, GridPlusOutline, ObjectsColumnOutline, SearchOutline, UsersGroupOutline } from 'flowbite-svelte-icons';

    const getSHA256Hash = async (input: string) => {
        const textAsBuffer = new TextEncoder().encode(input);
        const hashBuffer = await window.crypto.subtle.digest("SHA-256", textAsBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        const hash = hashArray
            .map((item) => item.toString(16).padStart(2, "0"))
            .join("");
        return hash;
    };

    let emailhash: string;

    $: {
        if ($auth) {
            getSHA256Hash($auth.email??'').then((hash) => {
                emailhash = hash;
            });
        }
    }
</script>

{#if $auth}
    <div class="flex absolute right-0 left-0" style="padding: 0 5rem 0 5rem;">
        <div class="flex-none w-214 pr-6">
            <Sidebar>
                <SidebarWrapper class="rounded-xl border border-gray-200 dark:border-gray-700 divide-gray-200 dark:divide-gray-700 shadow-md">
                    <SidebarGroup>
                        <h1 class="font-semibold text-gray-900 dark:text-white text-2xl ml-2 mb-4">Dashboard</h1>
                        <span class="block h-1/8"></span>
                        <div class="w-214 bg-white rounded-lg border border-gray-200 dark:bg-gray-800 dark:border-gray-600 divide-y divide-gray-200 dark:divide-gray-600">
                            <SidebarItem href="/dashboard" label="Overview">
                                <svelte:fragment slot="icon">
                                    <ObjectsColumnOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                            <SidebarItem href="/dashboard/notifications" label="Notifications">
                                <svelte:fragment slot="icon">
                                    <BellOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                            <SidebarItem href="/dashboard/analytics" label="Analytics">
                                <svelte:fragment slot="icon">
                                    <ChartLineUpOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                        </div>
                        <span class="block h-1"></span>
                        <h2 class="font-semibold text-gray-900 dark:text-white text-xl ml-2 mb-4">Manage</h2>
                        <span class="block h-1/8"></span>
                        <div class="w-214 bg-white rounded-lg border border-gray-200 dark:bg-gray-800 dark:border-gray-600 divide-y divide-gray-200 dark:divide-gray-600">
                            <SidebarItem href="/dashboard" label="Projects">
                                <svelte:fragment slot="icon">
                                    <GridPlusOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                            <SidebarItem href="/dashboard/notifications" label="Organizations">
                                <svelte:fragment slot="icon">
                                    <UsersGroupOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                            <SidebarItem href="/dashboard/analytics" label="Collections">
                                <svelte:fragment slot="icon">
                                    <FolderOpenOutline class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" />
                                </svelte:fragment>
                            </SidebarItem>
                        </div>
                    </SidebarGroup>
                </SidebarWrapper>
            </Sidebar>
        </div>
        <div class="flex-auto w-2/4">
            <Card size="lg" padding="xl" horizontal>
                <Avatar size="lg" src="https://www.gravatar.com/avatar/{emailhash}" border />
                <div class="flex flex-col items-left">
                    <h5 class="mb-1 text-xl font-medium text-gray-900 dark:text-white m-4">{$auth.username}</h5>
                    <span class="ml-4 text-sm text-gray-500 dark:text-gray-400 m-1">{$auth.email}</span>
                </div>
            </Card>
        </div>
    </div>
{/if}