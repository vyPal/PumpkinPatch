<script>
    import { Pagination, ImagePlaceholder, Button, Input, Checkbox, Popover } from 'flowbite-svelte';

    let helper = { start: 1, end: 2, total: 20 };

    import { page } from '$app/stores';
	import { ArrowUpRightFromSquareOutline, EnvelopeSolid, ExclamationCircleOutline, SearchOutline, SearchSolid } from 'flowbite-svelte-icons';

    $: activeUrl = $page.url.searchParams.get('page');
    let pages = [
        { name: "1", href: '/discover?page=1', active: false },
        { name: "2", href: '/discover?page=2', active: false },
        { name: "...", href: '/discover?page=3', active: false },
        { name: "9", href: '/discover?page=9', active: false },
        { name: "10", href: '/discover?page=10', active: false }
    ];

    $: {
        pages.forEach((page) => {
            let splitUrl = page.href.split('?');
            let queryString = splitUrl.slice(1).join('?');
            const hrefParams = new URLSearchParams(queryString);
            let hrefValue = hrefParams.get('page');
            if (hrefValue === activeUrl) {
                page.active = true;
            } else {
                page.active = false;
            }
        });
        pages = pages;
    }

    const previous = () => {
        alert('Previous btn clicked. Make a call to your server to fetch data.');
    };
    const next = () => {
        alert('Next btn clicked. Make a call to your server to fetch data.');
    };

    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
</script>

<div class="flex flex-col absolute right-0 left-0" style="padding: 0 5rem 0 5rem;">
    <div class="flex">
        <div class="flex-none w-214" style="padding-right: 25px;">
            <Sidebar>
                <SidebarWrapper class="rounded-xl border border-gray-200 dark:border-gray-700 divide-gray-200 dark:divide-gray-700 shadow-md">
                    <SidebarGroup>
                        <Input id="search" placeholder="Search plugins">
                            <SearchOutline slot="left" class="w-5 h-5 text-gray-500 dark:text-gray-400" />
                        </Input>
                        <span class="block h-1/8"></span>
                        <ul class="w-214 bg-white rounded-lg border border-gray-200 dark:bg-gray-800 dark:border-gray-600 divide-y divide-gray-200 dark:divide-gray-600">
                            <li style="padding: 5px 0 5px 5px;">
                                <Checkbox class="p-1" checked>All plugins</Checkbox>
                                <Checkbox class="p-1">Addons</Checkbox>
                                <Checkbox class="p-1">Economy</Checkbox>
                                <Checkbox class="p-1">Optimization</Checkbox>
                                <Checkbox class="p-1">World management</Checkbox>
                            </li>
                            <li style="padding: 5px 0 5px 5px;">
                                <Checkbox class="p-1" checked>
                                    v0.1.0-dev
                                    <button id="b1">
                                        <ExclamationCircleOutline class="w-5 h-5 ms-1.5" />
                                        <span class="sr-only">Warning</span>
                                    </button>
                                    <Popover triggeredBy="#b1" class="w-72 text-sm font-light text-gray-500 bg-white dark:bg-gray-800 dark:border-gray-600 dark:text-gray-400" placement="bottom-start" style="z-index: 5;">
                                        <div class="p-3 space-y-2">
                                            <h3 class="font-semibold text-gray-900 dark:text-white">Development version</h3>
                                            This version is under heavy development. The API may change at any time.
                                        </div>
                                    </Popover>
                                </Checkbox>
                            </li>
                        </ul>
                    </SidebarGroup>
                </SidebarWrapper>
            </Sidebar>
        </div>
        <div class="flex-auto w-3/4">
            {#each { length: 2 } }
                <ImagePlaceholder />
                <span class="block h-4"></span>
            {/each}
        </div>
    </div>
    <span class="block h-12"></span>
    <div class="flex flex-col items-center justify-center gap-2">
        <div class="text-sm text-gray-700 dark:text-gray-400">
            Showing <span class="font-semibold text-gray-900 dark:text-white">{helper.start}</span>
            to
            <span class="font-semibold text-gray-900 dark:text-white">{helper.end}</span>
            of
            <span class="font-semibold text-gray-900 dark:text-white">{helper.total}</span>
            Entries
        </div>
        <Pagination {pages}>
            <span slot="prev">Prev</span>
        </Pagination>
    </div>
</div>