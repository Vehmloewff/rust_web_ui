use rust_web_ui::*;

pub struct Example35;

pub struct Example35Props {}

impl Default for Example35Props {
	fn default() -> Example35Props {
		Example35Props { }
	}
}

impl Widget<'_> for Example35 {
	type Props = Example35Props;

	fn render(mut ctx: Ctx<'_>, props: Example35Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("section", |props| {
			props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white"), Style::PaddingX(24), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[Style::PaddingX(32)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("-z-10"), NoStyle::Noop("bg-[radial-gradient(45rem_50rem_at_top,theme(colors.indigo.100),white)]"), NoStyle::Noop("opacity-20")]);
			});
			props.child("3", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-1/2"), NoStyle::Noop("-z-10"), Style::MarginRight(64), NoStyle::Noop("w-[200%]"), NoStyle::Noop("origin-bottom-left"), NoStyle::Noop("skew-x-[-30deg]"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-xl"), NoStyle::Noop("shadow-indigo-600/10"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-indigo-50"), Screen::Small(&[Style::MarginRight(112)]), Screen::Large(&[Style::MarginRight(0)]), Screen::ExtraLarge(1, &[Style::MarginRight(64)]), Screen::ExtraLarge(1, &[NoStyle::Noop("origin-center")])]);
			});
			props.child("5", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Screen::Large(&[NoStyle::Noop("max-w-4xl")])]);
				
				props.child("1", Dynamic).run("img", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::Width(48)]);
					props.set_attribute("src", "https://tailwindui.com/img/logos/workcation-logo-indigo-600.svg");
					props.set_attribute("alt", "");
				});
				props.child("3", Dynamic).run("figure", |props| {
					props.styles(&[Style::MarginTop(40)]);
					
					props.child("1", Dynamic).run("blockquote", |props| {
						props.styles(&[NoStyle::Noop("text-center"), NoStyle::Noop("text-xl"), Style::FontSemibold, NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-2xl")]), Screen::Small(&[NoStyle::Noop("leading-9")])]);
						
						props.child("1", Dynamic).run("p", |props| {
							props.child("0", Label).run(|props| props.set_text("“Lorem ipsum dolor sit amet consectetur adipisicing elit. Nemo expedita voluptas culpa sapiente alias molestiae. Numquam corrupti in laborum sed rerum et corporis.”"));
						});
					});
					props.child("3", Dynamic).run("figcaption", |props| {
						props.styles(&[Style::MarginTop(40)]);
						
						props.child("1", Dynamic).run("img", |props| {
							props.styles(&[NoStyle::Noop("mx-auto"), Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full")]);
							props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
							props.set_attribute("alt", "");
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(16), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::SpaceX(12), NoStyle::Noop("text-base")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Judith Black"));
							});
							props.child("3", Icon).run(|props| {
								props.style(&[NoStyle::Noop("fill-gray-900")]);
							});
							props.child("5", Dynamic).run("div", |props| {
								props.styles(&[Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("CEO of Workcation"));
							});
						});
					});
				});
			});
		});
	}
}
