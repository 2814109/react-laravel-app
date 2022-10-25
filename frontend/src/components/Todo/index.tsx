import { FC } from "react";
import InputSection from "./InputSection";
import { Box } from "@chakra-ui/react";
import { Center } from "@chakra-ui/react";
import useGetTasks from "../../hooks/task/getTasks";
import TodoList from "./TodoTable";
const Todo: FC = () => {
  const { data } = useGetTasks();
  console.log(data);
  // indexで状態を管理　かつFectch処理　Fetch中はListだけSuspenseでWrapし、InputSection　は入力ボタンを非活性にする処理を入れる
  return (
    <>
      <Center>
        <Box w={"50%"} m={24}>
          <InputSection />
        </Box>
      </Center>
      <Center>
        <Box w={"50%"}>
          <TodoList />
        </Box>
      </Center>
    </>
  );
};
export default Todo;
