<script lang="ts">
  import init, { calculate } from './lib/backend'

  init().then(() => {
    console.log("Initialised wasm...")
    console.log(calculate('(10.3123^3)*3 + 2   '));
  })

  function press(button: string) {
    console.log("pressed: " + button)
  }
</script>

<main>
  <div class="flex justify-center items-center h-screen bg-gradient-to-br from-green-400 to-blue-500">
    <div class="bg-white bg-opacity-20 backdrop-blur-lg p-10 rounded-2xl">
      <div class="bg-white h-10 rounded-full text-right p-3 align-middle">e</div>
      <div class="grid grid-cols-5 grid-rows-4 gap-4 m-4">
        <button on:click={() => press('7')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">7</button>
        <button on:click={() => press('8')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">8</button>
        <button on:click={() => press('9')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">9</button>
        <button on:click={() => press('DEL')} class="duration-200 ease-in-out hover:bg-red-600 hover:shadow-xl shadow-md text-center bg-red-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">DEL</button>
        <button on:click={() => press('AC')} class="duration-200 ease-in-out hover:bg-red-600 hover:shadow-xl shadow-md text-center bg-red-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">AC</button>
        <button on:click={() => press('4')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">4</button>
        <button on:click={() => press('5')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">5</button>
        <button on:click={() => press('6')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">6</button>
        <button on:click={() => press('×')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">×</button>
        <button on:click={() => press('÷')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">÷</button>
        <button on:click={() => press('1')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">1</button>
        <button on:click={() => press('2')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">2</button>
        <button on:click={() => press('3')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">3</button>
        <button on:click={() => press('+')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">+</button>
        <button on:click={() => press('-')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">-</button>
        <button on:click={() => press('0')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">0</button>
        <button on:click={() => press('.')} class="duration-200 ease-in-out hover:bg-green-600 hover:shadow-xl shadow-md text-center bg-green-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">.</button>
        <button on:click={() => press('=')} class="duration-200 ease-in-out hover:bg-purple-600 hover:shadow-xl shadow-md text-center bg-purple-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">=</button>
        <button on:click={() => press('(')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">(</button>
        <button on:click={() => press(')')} class="duration-200 ease-in-out hover:bg-orange-600 hover:shadow-xl shadow-md text-center bg-orange-500 rounded-lg py-4 px-8 font-thin text-2xl text-white">)</button>
      </div>
    </div>
  </div>
</main>