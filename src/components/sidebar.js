export function toggleSidebar() {
    document.querySelector(".sidebar").classList.toggle("collapsed");

    document.querySelector("#sidebar-toggle").classList.toggle("collapsed");
    document.querySelector("#sidebar-toggle").textContent = document.querySelector("#sidebar-toggle").textContent === "❯" ? "❮" : "❯";

    document.querySelector("#canvas").classList.toggle("sidebar-collapsed");
}