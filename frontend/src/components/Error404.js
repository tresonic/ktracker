import { route } from "preact-router";
import { useEffect } from "preact/hooks";

export default function Error404() {
    useEffect(() => {
        setTimeout(() => {
            route("/");
        }, 2000);
    }, [])

    return (
        <div class="has-text-centered">
            <h1 class="is-size-1">Page not found</h1>
            <p>Redirecting...</p>
        </div>
    );
}