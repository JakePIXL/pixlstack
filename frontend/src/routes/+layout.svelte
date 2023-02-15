<script>
    import Navbar from "../components/Navbar.svelte";
  import "../app.postcss";
</script>

<div class="flex flex-col min-w-screen min-h-screen">
  <Navbar />
  <div class="flex-grow px-4">
    <slot />
  </div>
  <footer class="container mx-auto">
    <div class="row">
      <div class="col-md-12">
        <p class="text-center text-gray-500 p-4 font-signika font-semibold">
          Copyright &copy; 2023 Jake Evans. All Rights Reserved.
        </p>
      </div>
    </div>
  </footer>
</div>