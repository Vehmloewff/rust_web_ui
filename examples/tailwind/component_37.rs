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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Width(Size::Exact(0)), Style::Noop("min-h-[768px]"), Style::Noop("bg-white")]);
			
			// 
			//   This example requires updating your template:
			// 
			//   ```
			//   <html class="h-full">
			//   <body class="h-full">
			//   ```
			//   
			props.child("3", Dynamic).run("main", |mut props| {
				props.styles(&[Style::Noop("grid"), Style::Noop("min-h-full"), Style::Noop("place-items-center"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("text-center")]);
					
					props.child("1", Dynamic).run("p", |mut props| {
						props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("text-indigo-600")]);
						
						props.child("0", Label).run(|props| props.text("404"));
					});
					props.child("3", Dynamic).run("h1", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-5xl")])]);
						
						props.child("0", Label).run(|props| props.text("Page not found"));
					});
					props.child("5", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.text("Sorry, we couldn’t find the page you’re looking for."));
					});
					props.child("7", Dynamic).run("div", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-6")]);
						
						props.child("1", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(10)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
							
							props.child("0", Label).run(|props| props.text("Go back home"));
						});
						props.child("3", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Contact support"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
					});
				});
			});
		});
	}
}
