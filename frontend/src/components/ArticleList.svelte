<script>
	import ArticlesItem from "./ArticlesItem.svelte";
    import { PaginationItem } from 'flowbite-svelte';
  const previous = () => {
    alert('Previous btn clicked. Make a call to your server to fetch data.');
  };
  const next = () => {
    alert('Next btn clicked. Make a call to your server to fetch data.');
  };
</script>

<div class="flex flex-col gap-4 items-center">
    <ArticlesItem />
    <ArticlesItem />
    <ArticlesItem />
    <ArticlesItem />
    <div class="flex space-x-3 w-fit">
        <PaginationItem on:click={previous}>Previous</PaginationItem>
        <PaginationItem on:click={next}>Next</PaginationItem>
    </div>
</div>