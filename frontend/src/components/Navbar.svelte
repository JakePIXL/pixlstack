<script lang="ts">
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button, DarkMode } from 'flowbite-svelte'
    import Icon from '@iconify/svelte';
	import Logo from './Logo.svelte';
    import { page } from '$app/stores';
    let path: String;
    $: path = $page.url.pathname;
</script>
  
<Navbar let:hidden let:toggle color="none" class="px-4">
    <NavBrand href="/">
        <div class="mr-3 h-6 sm:h-9">
            <Logo />
        </div>
        <div class="flex flex-col gap-0 w-fit">
            <span class="md:text-xl font-semibold dark:text-slate-100 font-palanquin-dark h-fit">Jake <span class="text-blue-500">PIXL</span> Evans</span>
            <span class="font-semibold text-slate-600 text-xs md:text-sm font-palanquin-dark h-fit -mt-1.5">Backend Developer</span>
        </div>
    </NavBrand>
    <div class="flex md:order-2">
        <div class="flex flex-row gap-0.5 md:gap-2 justify-center items-center md:mr-2">
            <a href="https://www.linkedin.com/in/jakepixl/" target="_blank" rel="noreferrer" class="text-gray-500 dark:text-gray-400 hover:text-blue-500"><span class="sr-only">LinkedIn</span><Icon icon="mdi:linkedin" /></a>
            <a href="https://twitter.com/JakePIXL" target="_blank" rel="noreferrer" class="text-gray-500 dark:text-gray-400 hover:text-blue-500"><span class="sr-only">Twitter</span><Icon icon="mdi:twitter-box" /></a>
            <a href="https://tech.lgbt/@JakePIXL" rel="me" target="_blank" class="text-gray-500 dark:text-gray-400 hover:text-blue-500"><span class="sr-only">Mastodon</span><Icon icon="mdi:mastodon" /></a>
            <a href="https://github.com/JakePIXL" target="_blank" rel="noreferrer" class="text-gray-500 dark:text-gray-400 hover:text-blue-500"><span class="sr-only">Github</span><Icon icon="mdi:github-box" /></a>
        </div>
        <DarkMode btnClass="text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring-none rounded-lg text-sm p-2.5" />
        <NavHamburger on:click={toggle} />
    </div>
    <NavUl {hidden} class="font-roboto">
        <NavLi href="/" active={path === "/" ? true : false}>Home</NavLi>
        <NavLi href="/about" active={path === "/about" ? true : false}>About</NavLi>
        <NavLi href="/contact" active={path === "/contact" ? true : false}>Contact</NavLi>
    </NavUl>
</Navbar>