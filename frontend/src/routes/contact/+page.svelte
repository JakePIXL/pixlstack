<script lang="ts">
    import { Alert, Button, Card, Checkbox, Input, Label, Hr, Textarea } from "flowbite-svelte";
    import { Turnstile } from 'svelte-turnstile';
    import type { ActionData } from './$types';

    export let form: ActionData;
</script>
<div class="flex flex-col">
    <Card size="xl" class="format dark:format-invert min-w-full md:min-w-[500px] mx-auto">
        <h1 class="border-l-2 border-blue-500 pl-2 text-2xl font-palanquin-dark">Contact Me</h1>

        {#if form?.success}

            {#if form?.success === true}
            <Alert color="green" dismissable>
                <span slot="icon"><svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
                </span>
                Message sent successfully, I will contact you shortly.
            </Alert>
            {:else}
            <Alert color="red" dismissable>
                <span slot="icon"><svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
                </span>
                {form?.error.message}
            </Alert>
            {/if}
        {/if}

        <form method="POST" class="flex flex-col gap-4">
            <div>
                <Label for="name">Name</Label>
                <Input id="name" name="name" type="text" />
            </div>
            <div>
                <Label for="email">Email</Label>
                <Input id="email" name="email" type="email"/>
            </div>
            <div>
                <Label for="company">Company</Label>
                <Input id="company" name="company" type="text"/>
            </div>
            <div>
                <Label for="message">Your Message</Label>
                <Textarea id="message" name="message" />
            </div>
            <div class="flex flex-row items-center justify-center gap-2">
                <Hr class="my-4 mx-auto md:my-10" width="w-full" height="h-1"/>
                <Turnstile siteKey="0x4AAAAAAACcHDmBIuD4hnu9" forms={true} />
                <Hr class="my-4 mx-auto md:my-10" width="w-full" height="h-1"/>
            </div>
            <div class="flex flex-row items-center gap-4">
                <Card class="w-full p-2 h-full shadow-none" padding="none">
                    <Checkbox id="urgency" value="false" class="w-full">Need a response quickly?</Checkbox>
                </Card>
                <Button type="submit" class="w-1/4 h-full">Submit</Button>
            </div>
        </form>
    </Card>
</div>