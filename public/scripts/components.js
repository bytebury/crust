// APP DRAWER
function openAppDrawer() {
	const drawer = document.getElementById("app_drawer");
	const underlay = document.getElementById("app_drawer_underlay");
	drawer.style.transform = "translateX(0)";
	underlay.style.display = "block";
	hideTooltip();
}

function closeAppDrawer() {
	const drawer = document.getElementById("app_drawer");
	const underlay = document.getElementById("app_drawer_underlay");
	drawer.style.transform = "translateX(100%)";
	underlay.style.display = "none";
}

// MODAL
document.addEventListener("closeModal", function () {
	closeModal();
});

document.addEventListener("htmx:afterSwap", function (evt) {
	if (evt.target.id === "modal") {
		document.getElementById("modal_wrapper").style.display = "flex";
	}
});

function closeModal() {
	const modal = document.getElementById("modal_wrapper");

	modal.classList.add("closing");

	modal.addEventListener("animationend", function handleAnimationEnd() {
		modal.classList.remove("closing");
		modal.style.display = "none";
		modal.removeEventListener("animationend", handleAnimationEnd);
	});
}

// TOOLTIPS
let activeTooltip = null;

function showTooltip(event) {
	activeTooltip = event.currentTarget;
	tooltip.innerHTML = activeTooltip.dataset.tooltip;

	tooltip.classList.add("measure");
	const tooltipRect = tooltip.getBoundingClientRect();
	tooltip.classList.remove("measure");

	const rect = activeTooltip.getBoundingClientRect();
	const scrollX = window.scrollX;
	const windowWidth = window.innerWidth;

	const finalTop = rect.bottom + window.scrollY + 4;
	const preferredLeft = rect.left + scrollX + rect.width / 2 -
		tooltipRect.width / 2;

	let finalLeft;

	if (preferredLeft < scrollX) {
		finalLeft = rect.left + scrollX;
	} else if (preferredLeft + tooltipRect.width > scrollX + windowWidth) {
		finalLeft = rect.right + scrollX - tooltipRect.width;
	} else {
		finalLeft = preferredLeft;
	}

	tooltip.style.top = `${finalTop}px`;
	tooltip.style.left = `${finalLeft}px`;

	tooltip.classList.add("show");
}

function hideTooltip() {
	tooltip.classList.remove("show");
	activeTooltip = null;
}

function initTooltips() {
	for (const element of document.querySelectorAll("[data-tooltip")) {
		element.addEventListener("mouseenter", showTooltip);
		element.addEventListener("mouseleave", hideTooltip);
	}
}

document.addEventListener("DOMContentLoaded", function () {
	// Initialize tooltips when the page loads.
	initTooltips();
});

// Every time that there's a swap, we need to reinitialize the tooltips.
document.body.addEventListener("htmx:afterSwap", function (event) {
	hideTooltip();
	initTooltips();
});
