import { Button, ConfigProvider } from "antd";
import { useSetAtom } from "jotai";
import { useCallback, useEffect } from "react";
import { useAuth } from "./api/auth";
import useExplorerDispatcher from "./api/explorer";
import { useMe } from "./api/user";
import { ReactionPicker } from "./components/RactionPicker";
import { SpeakerPhoneButton } from "./components/SpeakerPhoneButton";
import { Timeline } from "./components/Timeline";
import Canvas from "./pixi/Canvas";
import { User } from "./schema2/user";
import dispatcherAtom from "./state/dispatcher";
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

  const loginOnClick = useCallback(async () => {
    const res = await authTrigger();
    window.location.href = res.location;
  }, [authTrigger]);

  useEffect(
    () => setMe({ id: resUser?.user?.id, name: resUser?.user?.name } as User),
    [resUser, setMe],
  );

  if (error) {
    console.log(error);
  }

  if (isLoading) {
    return <div>loading...</div>;
  }
  if (!resUser) {
    return <Button onClick={loginOnClick}>ログイン</Button>;
  }

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
            flex
            gap-2
          "
        >
          <ReactionPicker />
          <SpeakerPhoneButton />
        </div>
        <Timeline />
      </ConfigProvider>
    </div>
  );
};

export default App;
