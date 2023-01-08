<script lang="ts">
  import { onMount } from "svelte";

  onMount(async () => {
    const sidebar = document.getElementById("sidebar");

    let wrapper = document.createElement("div");
    wrapper.style.backgroundColor = "black";
    wrapper.style.borderRadius = "1em";
    wrapper.style.width = "1.8em";
    wrapper.style.height = "1.8em";
    wrapper.style.margin = "0.1em";
    wrapper.style.position = "absolute";
    wrapper.style.zIndex = "1000";

    let stone = document.createElementNS("http://www.w3.org/2000/svg", "svg"); //Create a path in SVG's namespace

    stone.setAttribute("width", "86");
    stone.setAttribute("height", "86");
    stone.setAttribute("color", "black");
    stone.setAttribute("position", "absolute");
    stone.setAttribute("top", "0");
    stone.setAttribute("right", "0");
    stone.setAttribute("bottom", "0");
    stone.setAttribute("left", "0");
    stone.setAttribute("z-index", "100");

    let circle = document.createElementNS(
      "http://www.w3.org/2000/svg",
      "circle"
    );
    circle.setAttribute("cx", "15px");
    circle.setAttribute("cy", "15px");
    circle.setAttribute("r", "10px");
    circle.setAttribute("fill", "black");

    stone.appendChild(circle);

    sidebar.addEventListener("click", (e) => {
      console.log("circle clicked - enlarging");

      sidebar.appendChild(wrapper);
    });
  });
</script>

<main>
  <div id="sidebar" width="200" height="600">Placeholder</div>
</main>

<style>
  #sidebar {
    background-color: aqua;
  }
</style>
