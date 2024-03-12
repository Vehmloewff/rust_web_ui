use rust_web_ui::*;

pub struct Example37;

pub struct Example37Props {}

impl Default for Example37Props {
	fn default() -> Example37Props {
		Example37Props { }
	}
}

impl Widget<'_> for Example37 {
	type Props = Example37Props;

	fn render(mut ctx: Ctx<'_>, props: Example37Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Width(0), NoStyle::Noop("min-h-[768px]"), NoStyle::Noop("bg-white")]);
			
			// 
			//   This example requires updating your template:
			// 
			//   ```
			//   <html class="h-full">
			//   <body class="h-full">
			//   ```
			//   
			props.child("3", Dynamic).run("main", |props| {
				props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("min-h-full"), NoStyle::Noop("place-items-center"), NoStyle::Noop("bg-white"), Style::PaddingX(24), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("text-center")]);
					
					props.child("1", Dynamic).run("p", |props| {
						props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("text-indigo-600")]);
						
						props.child("0", Label).run(|props| props.set_text("404"));
					});
					props.child("3", Dynamic).run("h1", |props| {
						props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-5xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Page not found"));
					});
					props.child("5", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.set_text("Sorry, we couldn’t find the page you’re looking for."));
					});
					props.child("7", Dynamic).run("div", |props| {
						props.styles(&[Style::MarginTop(40), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-6")]);
						
						props.child("1", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(14), Style::PaddingY(10), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
							
							props.child("0", Label).run(|props| props.set_text("Go back home"));
						});
						props.child("3", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Contact support"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
					});
				});
			});
		});
	}
}
