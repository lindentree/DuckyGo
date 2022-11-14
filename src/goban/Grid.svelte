<script lang="ts">
//   const drawGrid = (context) => {
//     return <rect>
//   }

const testArray = [...Array(18).keys()];
</script>

<main>
    <div class="shudan-grid">
        <svg
        class=shudan-lines,
        style="
          position: absolute;
          top: 0;
          left: 0,
          width: 100%;
          height: 100%;
          pointerEvents: none;
          zIndex: 2
          "
        >
            {#each testArray as rows, i}
            <line
            x1={i * 16}
            x2={i * 16}
            y1={0}
            y2={(i-1) * 16}
            class="shudan-line"
            stroke="black"
            width=0.5
            ></line>
            <line
            x1={0}
            x2={(i - 1) * 16}
            y1={i * 16}
            y2={i * 16}
            class="shudan-line"
            stroke="black"
            width=0.5
            ></line>
        
            {/each}
        </svg>
     
    </div>
       

</main>

<style>
    /* Lines & arrows */

.shudan-line,
.shudan-arrow {
  fill: none;
  stroke: var(--shudan-board-foreground-color);
  stroke-width: .11em;
}
   
</style>