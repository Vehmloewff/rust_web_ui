use rust_web_ui::*;

pub struct Example28;

pub struct Example28Props {}

impl Default for Example28Props {
	fn default() -> Example28Props {
		Example28Props { }
	}
}

impl Widget<'_> for Example28 {
	type Props = Example28Props;

	fn render(mut ctx: Ctx<'_>, props: Example28Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("overflow-hidden"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("grid"), Style::Noop("max-w-2xl"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::OnScreen(Screen::Small, &[Style::Noop("gap-y-20")]), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Large, &[Style::PaddingRight(Size::Exact(32))]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(16))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("max-w-lg")])]);
							
							props.child("1", Dynamic).run("h2", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.text("Deploy faster"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
								
								props.child("0", Label).run(|props| props.text("A better workflow"));
							});
							props.child("5", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
							});
							props.child("7", Dynamic).run("dl", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Noop("max-w-xl"), Style::SpaceY(Size::Exact(32)), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67)), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")])]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(36))]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Noop("absolute"), Style::Noop("left-1"), Style::Noop("top-1"), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.text("Push to deploy."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(36))]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Noop("absolute"), Style::Noop("left-1"), Style::Noop("top-1"), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.text("SSL certificates."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo."));
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("relative"), Style::PaddingLeft(Size::Exact(36))]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Noop("absolute"), Style::Noop("left-1"), Style::Noop("top-1"), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.text("Database backups."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.text("Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."));
									});
								});
							});
						});
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.set_attribute("src", "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png");
						props.set_attribute("alt", "Product screenshot");
						props.styles(&[Style::Noop("w-[48rem]"), Style::Noop("max-w-none"), Style::Noop("rounded-xl"), Style::Noop("shadow-xl"), Style::Noop("ring-1"), Style::Noop("ring-gray-400/10"), Style::OnScreen(Screen::Small, &[Style::Noop("w-[57rem]")]), Style::OnScreen(Screen::Medium, &[Style::Noop("-ml-4")]), Style::OnScreen(Screen::Large, &[Style::Noop("-ml-0")])]);
						props.set_attribute("width", "2432");
						props.set_attribute("height", "1442");
					});
				});
			});
		});
	}
}
