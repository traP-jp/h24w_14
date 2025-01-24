use std::fmt;

impl fmt::Debug for super::TraqBotConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TraqBotConfig")
            .field("bot_id", &self.bot_id)
            .field("bot_user_id", &self.bot_user_id)
            .field("verification_token", &"REDACTED")
            .field("access_token", &"REDACTED")
            .finish()
    }
}

impl super::TraqBotConfig {
    pub fn builder() -> Builder {
        Builder {
            bot_id: (),
            bot_user_id: (),
            verification_token: (),
            access_token: (),
        }
    }
}

pub struct Builder<BotId = (), BotUserId = (), VerificationToken = (), AccessToken = ()> {
    bot_id: BotId,
    bot_user_id: BotUserId,
    verification_token: VerificationToken,
    access_token: AccessToken,
}

impl<BotId, BotUserId, VerificationToken, AccessToken>
    Builder<BotId, BotUserId, VerificationToken, AccessToken>
{
    pub fn bot_id(
        self,
        value: impl Into<String>,
    ) -> Builder<String, BotUserId, VerificationToken, AccessToken> {
        let Self {
            bot_id: _,
            bot_user_id,
            verification_token,
            access_token,
        } = self;
        Builder {
            bot_id: value.into(),
            bot_user_id,
            verification_token,
            access_token,
        }
    }

    pub fn bot_user_id(
        self,
        value: impl Into<String>,
    ) -> Builder<BotId, String, VerificationToken, AccessToken> {
        let Self {
            bot_id,
            bot_user_id: _,
            verification_token,
            access_token,
        } = self;
        Builder {
            bot_id,
            bot_user_id: value.into(),
            verification_token,
            access_token,
        }
    }

    pub fn verification_token(
        self,
        value: impl Into<String>,
    ) -> Builder<BotId, BotUserId, String, AccessToken> {
        let Self {
            bot_id,
            bot_user_id,
            verification_token: _,
            access_token,
        } = self;
        Builder {
            bot_id,
            bot_user_id,
            verification_token: value.into(),
            access_token,
        }
    }

    pub fn access_token(
        self,
        value: impl Into<String>,
    ) -> Builder<BotId, BotUserId, VerificationToken, String> {
        let Self {
            bot_id,
            bot_user_id,
            verification_token,
            access_token: _,
        } = self;
        Builder {
            bot_id,
            bot_user_id,
            verification_token,
            access_token: value.into(),
        }
    }
}

impl Builder<String, String, String, String> {
    pub fn build(self) -> super::TraqBotConfig {
        let Self {
            bot_id,
            bot_user_id,
            verification_token,
            access_token,
        } = self;
        super::TraqBotConfig {
            bot_id,
            bot_user_id,
            verification_token,
            access_token,
        }
    }
}
