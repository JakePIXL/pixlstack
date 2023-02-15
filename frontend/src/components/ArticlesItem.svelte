<script>
    import { Card, Badge } from "flowbite-svelte";
    import Icon from '@iconify/svelte';
</script>

<Card href="/" size="xl" class="group gap-2 min-w-full font-roboto">
    <div class="flex flex-row justify-between border-b pb-2 border-gray-300 dark:border-gray-700 dark:group-hover:border-gray-500 items-center">
        <h1 class="font-semibold text-2xl font-palanquin-dark">Title</h1>
        <Badge href="/" border>Category</Badge>
    </div>
    <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    </p>
    <div class="flex flex-row justify-between item-center">
        <Badge border color="dark">
            <Icon icon="mdi:clock-time-five-outline" class="w-3 h-3 mr-1" />
            Time
        </Badge>
        <Badge border color="dark">
            <Icon icon="mdi:chart-bar" class="w-3 h-3 mr-1" />
            Views
        </Badge>
    </div>
</Card>