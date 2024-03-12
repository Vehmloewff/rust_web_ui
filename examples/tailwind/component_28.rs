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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("grid"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), Screen::Small(&[NoStyle::Noop("gap-y-20")]), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Screen::Large(&[Style::PaddingRight(32)]), Screen::Large(&[Style::PaddingTop(16)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Screen::Large(&[NoStyle::Noop("max-w-lg")])]);
							
							props.child("1", Dynamic).run("h2", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.set_text("Deploy faster"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
								
								props.child("0", Label).run(|props| props.set_text("A better workflow"));
							});
							props.child("5", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
							});
							props.child("7", Dynamic).run("dl", |props| {
								props.styles(&[Style::MarginTop(40), NoStyle::Noop("max-w-xl"), Style::SpaceY(32), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67)), Screen::Large(&[NoStyle::Noop("max-w-none")])]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(36)]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1"), NoStyle::Noop("top-1"), Style::Width(20), Style::Width(20), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.set_text("Push to deploy."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |props| {
										props.styles(&[NoStyle::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.set_text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(36)]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1"), NoStyle::Noop("top-1"), Style::Width(20), Style::Width(20), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.set_text("SSL certificates."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |props| {
										props.styles(&[NoStyle::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.set_text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo."));
									});
								});
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(36)]);
									
									props.child("1", Dynamic).run("dt", |props| {
										props.styles(&[NoStyle::Noop("inline"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1"), NoStyle::Noop("top-1"), Style::Width(20), Style::Width(20), NoStyle::Noop("text-indigo-600")]);
										});
										props.child("2", Label).run(|props| props.set_text("Database backups."));
									});
									//  space 
									props.child("5", Dynamic).run("dd", |props| {
										props.styles(&[NoStyle::Noop("inline")]);
										
										props.child("0", Label).run(|props| props.set_text("Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."));
									});
								});
							});
						});
					});
					props.child("3", Dynamic).run("img", |props| {
						props.set_attribute("src", "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png");
						props.set_attribute("alt", "Product screenshot");
						props.styles(&[NoStyle::Noop("w-[48rem]"), NoStyle::Noop("max-w-none"), NoStyle::Noop("rounded-xl"), NoStyle::Noop("shadow-xl"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-400/10"), Screen::Small(&[NoStyle::Noop("w-[57rem]")]), Screen::Medium(&[NoStyle::Noop("-ml-4")]), Screen::Large(&[NoStyle::Noop("-ml-0")])]);
						props.set_attribute("width", "2432");
						props.set_attribute("height", "1442");
					});
				});
			});
		});
	}
}
