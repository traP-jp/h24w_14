import { Button, ConfigProvider } from "antd";
import { useSetAtom } from "jotai";
import { useEffect } from "react";
import useExplorerDispatcher from "./api/explorer";
import { StampPicker } from "./components/StampPicker";
import { Timeline } from "./components/Timeline";
import Canvas from "./pixi/Canvas";
import dispatcherAtom from "./state/dispatcher";
import { useMe } from "./api/user";
import { User } from "./schema2/user";
import { useAuth } from "./api/auth";
import meAtom from "./state/me";

const App = () => {
  const dispatcher = useExplorerDispatcher();
  const setDispatcher = useSetAtom(dispatcherAtom);
  useEffect(() => {
    setDispatcher(() => dispatcher);
  }, [dispatcher, setDispatcher]);
  const { data: resUser, error, isLoading } = useMe();
  const { trigger: authTrigger } = useAuth();
  const setMe = useSetAtom(meAtom);

  const loginOnClick = async () => {
    const res = await authTrigger();
    window.location.href = res.location;
  };

  if (error) {
    console.log(error);
  }

  if (isLoading) {
    return <div>loading...</div>;
  }
  if (!resUser) {
    return <Button onClick={loginOnClick}>ログイン</Button>;
  }

  setMe({ id: resUser.user?.id, name: resUser.user?.name } as User);

  return (
    <div className="flex">
      <Canvas />
      <ConfigProvider
        theme={{
          token: {
            colorPrimary: "#a0d911",
          },
        }}
      >
        <div
          className="
            absolute
            left-1/2
            -translate-x-1/2
            bottom-4
          "
        >
          <StampPicker />
        </div>
        <Timeline />
      </ConfigProvider>
    </div>
  );
};

export default App;
