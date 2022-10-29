import { FC, Suspense } from "react";
import Todo from "./components/Todo";
const App: FC = () => {
  return (
    <Suspense fallback={<>loading</>}>
      <Todo />
    </Suspense>
  );
};

export default App;
