import { FC } from "react";
import {
  Button,
  Flex,
  Input,
  Spacer,
  Box,
  FormControl,
  FormLabel,
} from "@chakra-ui/react";

const InputSection: FC = () => {
  return (
    <FormControl isRequired>
      <Flex>
        <Box w={"75%"}>
          <FormLabel>Title</FormLabel>
          <Input placeholder="todo title" />
        </Box>
        <Spacer />
        <Box w={"20%"} mt={"auto"}>
          <Button colorScheme="teal">登録</Button>
        </Box>
      </Flex>
    </FormControl>
  );
};

export default InputSection;
