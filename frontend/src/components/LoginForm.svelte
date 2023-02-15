<script>
    import { Button, Card, Checkbox, Input, Label, Hr, Textarea } from "flowbite-svelte";
</script>

<Card size="xl" class="format dark:format-invert min-w-full md:min-w-[500px] mx-auto">
    <h1 class="border-l-2 border-blue-500 pl-2 text-2xl font-palanquin-dark">Login</h1>
    <form action="" class="flex flex-col gap-4">
        <div>
            <Label for="email">Email</Label>
            <Input id="email" />
        </div>
        <Button type="submit" class="w-1/4 h-full">Login</Button>
    </form>
</Card>