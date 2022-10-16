import { FC } from "react";
import InputSection from "./InputSection";
import { Box } from "@chakra-ui/react";
import { Center } from "@chakra-ui/react";
const Todo: FC = () => {
  return (
    <Center>
      <Box w={"50%"} m={24}>
        <InputSection />
      </Box>
    </Center>
  );
};
export default Todo;
