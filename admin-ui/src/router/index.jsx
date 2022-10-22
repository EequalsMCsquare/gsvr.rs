import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ViewClient from "../views/client";
import ViewBenchmark from "../views/benchmark"
import ViewSetting from "../views/setting";

const router = createBrowserRouter([
  {
    path: "/",
    element: <ViewClient />
  },
  {
    path: "/clients",
    element: <ViewClient />
  },
  {
    path: "/benchmark",
    element: <ViewBenchmark />
  },
  {
    path: "/setting",
    element: <ViewSetting/>
  }
]);

function Router() {
    return (
        <RouterProvider router={router} />
    )
}
export default Router;