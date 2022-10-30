import { FC, useState } from "react";
import {
  Button,
  Flex,
  Input,
  Spacer,
  Box,
  FormControl,
  FormLabel,
} from "@chakra-ui/react";
import usePost from "../../hooks/task/usePost";
const InputSection: FC = () => {
  const { request } = usePost();
  const [inputValue, setInputValue] = useState<string>("");

  console.log("input" + inputValue);
  return (
    <FormControl isRequired>
      <Flex>
        <Box w={"75%"}>
          <FormLabel>Title</FormLabel>
          <Input
            placeholder="todo title"
            onChange={(event) => {
              setInputValue(event.currentTarget.value);
            }}
          />
        </Box>
        <Spacer />
        <Box w={"20%"} mt={"auto"}>
          <Button
            colorScheme="teal"
            onClick={() => {
              request(inputValue);
            }}
          >
            登録
          </Button>
        </Box>
      </Flex>
    </FormControl>
  );
};

export default InputSection;
