<script>
    import { appWindow } from '@tauri-apps/api/window';

    // Close the window when the close button is clicked
    function handleClose() {
        appWindow.close();
    }
</script>

<style>
    .titlebar {
        display: flex;
        justify-content: flex-end;
        align-items: center;
        height: 30px;
        background-color: #333;
        color: white;
        padding: 0 10px;
        width: 100%;
        user-select: none;
        box-sizing: border-box;
    }

    .close-button {
        cursor: pointer;
    }

    .close-button:hover {
        color: #ff3e00;
    }
</style>

<div data-tauri-drag-region class="titlebar">
    <span class="close-button" on:click={handleClose}>X</span>
</div>
