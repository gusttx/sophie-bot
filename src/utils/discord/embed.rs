use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};

pub struct EmbedField(String, String, bool);

pub struct Embed {
    color: u32,
    title: String,
    title_url: Option<String>,
    fields: Option<Vec<EmbedField>>,
    description: Option<String>,
    footer: Option<String>,
    small_image: Option<String>,
    large_image: Option<String>,
}

impl Embed {
    pub fn new(color: u32, title: impl Into<String>) -> Self {
        Self {
            color,
            title: title.into(),
            title_url: None,
            fields: None,
            description: None,
            footer: None,
            small_image: None,
            large_image: None,
        }
    }

    pub fn url(mut self, title_url: impl Into<String>) -> Self {
        self.title_url = Some(title_url.into());
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields
            .get_or_insert_with(Vec::new)
            .push(EmbedField(name.into(), value.into(), false));
        self
    }

    pub fn optional_field(self, name: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        if let Some(value) = value {
            return self.field(name, value);
        }
        self
    }

    pub fn fields(mut self, fields: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.fields = Some(
            fields
                .into_iter()
                .map(|(name, value)| EmbedField(name.into(), value.into(), false))
                .collect(),
        );
        self
    }

    pub fn inline_field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields
            .get_or_insert_with(Vec::new)
            .push(EmbedField(name.into(), value.into(), true));
        self
    }

    pub fn optional_inline_field(
        self,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        if let Some(value) = value {
            return self.inline_field(name, value);
        }
        self
    }

    pub fn desc(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn small_image(mut self, small_image: impl Into<String>) -> Self {
        self.small_image = Some(small_image.into());
        self
    }

    pub fn large_image(mut self, large_image: impl Into<String>) -> Self {
        self.large_image = Some(large_image.into());
        self
    }
}

impl Into<CreateEmbed> for Embed {
    fn into(self) -> CreateEmbed {
        let mut embed = CreateEmbed::new().color(self.color).title(&self.title);

        if let Some(title_url) = &self.title_url {
            embed = embed.url(title_url);
        }

        if let Some(fields) = &self.fields {
            embed = embed.fields(fields.iter().map(|f| (&f.0, &f.1, f.2)));
        }

        if let Some(desc) = &self.description {
            embed = embed.description(desc);
        }

        if let Some(footer) = &self.footer {
            embed = embed.footer(CreateEmbedFooter::new(footer));
        }

        if let Some(small_image) = &self.small_image {
            embed = embed.thumbnail(small_image);
        }

        if let Some(large_image) = &self.large_image {
            embed = embed.image(large_image);
        }

        embed
    }
}
