import { Route, route } from 'preact-router';
import { useEffect } from 'preact/hooks';

import authService from "../services/auth.service";

export default function AuthenticatedRoute(props) {
    const isLoggedIn = authService.getCurrentUser() != null;

    // only redirect once we've cleared the screen:
    useEffect(() => {
        if (!isLoggedIn) {
            route('/about', true);
        }
    }, [isLoggedIn]);

    // not logged in, render nothing:
    if (!isLoggedIn) return null;

    return <Route {...props} />;
}